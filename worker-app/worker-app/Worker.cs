using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;

namespace worker_app
{
    public class Worker : BackgroundService
    {
        private readonly ILogger<Worker> _logger;
        private readonly IHttpClientFactory _httpClientFactory;

        public Worker(ILogger<Worker> logger, IHttpClientFactory httpClientFactory)
        {
            _logger = logger;
            _httpClientFactory = httpClientFactory;
        }

        protected override async Task ExecuteAsync(CancellationToken stoppingToken)
        {
            while (!stoppingToken.IsCancellationRequested)
            {
                Console.WriteLine("hello world!");
                await Task.Delay(5000, stoppingToken);
            }

        }

        public override async Task StopAsync(CancellationToken cancellationToken)
        {
            var client = _httpClientFactory.CreateClient();
            _ = await client.GetAsync("http://localhost:8080/api/stopasync/workers"); //hardcoded for demo; we can place this in separate file.
            await base.StopAsync(cancellationToken);
        }

        public override async Task StartAsync(CancellationToken cancellationToken)
        {
            var client = _httpClientFactory.CreateClient();
            _ = await client.GetAsync("http://localhost:8080/api/startasync/workers "); //hardcoded for demo; we can place this in separate file.
            await base.StartAsync(cancellationToken);
        }


    }
}
