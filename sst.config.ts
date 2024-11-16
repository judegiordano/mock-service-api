
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
      PORT: '80'
    }
    const vpc = new sst.aws.Vpc('ServiceMockerVpc', {
      nat: 'ec2',
      bastion: true,
    });
    const cluster = new sst.aws.Cluster('ServiceMockerCluster', { vpc });
    const service = cluster.addService('ServiceMockerService', {
      scaling: { min: 1, max: 1 },
      image: {
        context: './',
        dockerfile: 'Dockerfile',
      },
      logging: { retention: '1 week' },
      memory: '2 GB',
      cpu: '1 vCPU',
      loadBalancer: {
        domain: `api.servicemocker.${domain}`,
        ports: [
          { listen: '80/http' },
          { listen: '443/https', forward: '80/http' }
        ],
      },
      environment,
    });
    return {
      api: service.url
    }
  },
});
