using AngleSharp;
using AngleSharp.Dom;
using System;
using System.Collections.Generic;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

namespace Wyrm
{
    class Program
    {
        static async Task Main(string[] args)
        {
            var bookUrl = new UriBuilder(args[0]);

            var config = Configuration.Default
                .WithRequesters() // this line is important!! it allows the correct proxy settings to be "injected" into the AngleSharp browsing context.
                .WithDefaultLoader()
                .WithDefaultCookies();

            var titlePageRequest = BrowsingContext.New(config)
                .OpenAsync(bookUrl.ToString());

            var outputLocation = args[1];
            
            if (Directory.Exists(outputLocation))
            {
                new DirectoryInfo(outputLocation).Delete(recursive: true);
            }

            Directory.CreateDirectory(outputLocation);

            using var titlePage = await titlePageRequest.ConfigureAwait(false);

            var bookTitle = titlePage.QuerySelector("div.fic-header h1.font-white")?.TextContent.Trim() ?? throw new Exception("Unable to locate book title.");

            var chapterLinks = titlePage.QuerySelectorAll("table#chapters > tbody > tr.chapter-row > td:nth-child(1) > a");

            var chapterScrapingTasks = new Task[chapterLinks.Length];

            var chapterIndex = 0;
            foreach (var anchorTag in chapterLinks)
            {
                var chapterUrl = new Uri(bookUrl.Uri, anchorTag.Attributes["href"].Value);

                chapterScrapingTasks[chapterIndex++] = ScrapeChapterAsync(chapterUrl, Path.Combine(outputLocation, $"chapter-{chapterIndex}.html"), config);
            }

            Task.WaitAll(chapterScrapingTasks);
        }

        private static async Task ScrapeChapterAsync(Uri Link, string outputLocation, IConfiguration browserConfiguration)
        {
            var chapterUrl = Link.ToString();

            using var chapter = await BrowsingContext.New(browserConfiguration)
                .OpenAsync(chapterUrl)
                .ConfigureAwait(false);

            var chapterTitle = chapter.Head.QuerySelector("title") ?? throw new Exception("Unable to locate chapter title.");

            var chapterHeadingContent = chapter.Body.QuerySelector("div.fic-header h1.font-white")?.TextContent.Trim() ?? throw new Exception("Unable to locate chapter heading.");

            var chapterContent = chapter.Body.QuerySelector("div.chapter-inner.chapter-content") ?? throw new Exception("Unable to locate chapter content.");

            var authorNote = chapter.Body.QuerySelector("div.author-note");

            var chapterOutput = await GetNewHtmlPageAsync(browserConfiguration).ConfigureAwait(false);

            var chapterCharSet = chapterOutput.CreateElement("meta");
            chapterCharSet.SetAttribute("charset", "utf-8");
            chapterOutput.Head.AppendElement(chapterCharSet);

            chapterOutput.Head.AppendElement(chapterTitle);

            var chapterHeading = chapterOutput.CreateElement("h1");
            chapterHeading.ClassName = "chapter";
            chapterHeading.TextContent = chapterHeadingContent;
            chapterOutput.Body.AppendElement(chapterHeading);

            chapterOutput.Body.AppendElement(chapterContent);

            if (authorNote is null)
            {
                // do nothing
                // i wrote this if statement weird so i come back and change it to use "is not null" once i am using the correct version of C#
            }
            else
            {
                chapterOutput.Body.AppendElement(authorNote);
            }

            using var fileWriter = File.Create(outputLocation);

            await chapterOutput.ToHtmlAsync(fileWriter);
        }

        private static async Task<IDocument> GetNewHtmlPageAsync(IConfiguration browserConfiguration)
        {
            var html = @"<html></html>";
            var doc = await BrowsingContext.New(browserConfiguration).OpenAsync(req => req.Content(html));

            return doc;
        }
    }
}
