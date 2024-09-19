
/// <reference path='./.sst/platform/config.d.ts' />

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

    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      runtime: 'provided.al2023',
      bundle: 'target/lambda/api',
      memory: '! GB',
      timeout: '10 minutes',
      architecture: "arm64",
      url: { cors: true, allowCredentials: true },
      logging: {
        retention: '1 week',
        format: 'json'
      },
      environment,
    });

    return {
      url: api.url,
    }
  },
});
