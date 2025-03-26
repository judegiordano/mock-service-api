
/// <reference path='./.sst/platform/config.d.ts' />

const domain = 'judethings.com'

export default $config({
  app(input) {
    return {
      name: 'service-mocker',
      removal: 'remove',
      home: 'aws',
      providers: {
        aws: { region: 'us-east-1' }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;
    const environment = {
      STAGE: stage,
      LOG_LEVEL: process.env.LOG_LEVEL,
      MONGO_URI: process.env.MONGO_URI,
    }

    const func = new sst.aws.Function('ServiceMockerApi', {
      runtime: 'provided.al2023',
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      memory: '1 GB',
      timeout: '1 minute',
      architecture: 'arm64',
      url: { cors: { allowCredentials: true } },
      logging: { retention: '1 week', format: 'json' },
      environment,
    });

    const router = new sst.aws.Router('ServiceMockerRouter', {
      routes: { '/*': func.url },
      domain: {
        name: `api.mock.${domain}`,
        redirects: [`www.api.mock.${domain}`]
      }
    })
    return {
      function: func.url,
      url: router.url,
    }
  },
});
