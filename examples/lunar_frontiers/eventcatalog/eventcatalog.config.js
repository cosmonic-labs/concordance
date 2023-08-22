const path = require('path');

module.exports = {
  title: 'Lunar Frontiers',
  tagline: 'An event sourced application example',
  organizationName: 'Cosmonic',
  projectName: 'Lunar Frontiers',
  editUrl: 'https://github.com/cosmonic/concordance',
  trailingSlash: true,
  primaryCTA: {
    label: 'Explore Events',
    href: '/events'
  },
  // generators: [
  //   [
  //      '@eventcatalog/plugin-doc-generator-asyncapi',
  //     {
  //       // path to your AsyncAPI files
  //       pathToSpec: [path.join(__dirname, '../../lunar_frontiers/rover_aggregate.yaml')],

  //       // version events if already in catalog (optional)
  //       versionEvents: false
  //     },
  //   ],
  //    [
  //      '@eventcatalog/plugin-doc-generator-asyncapi',
  //     {
  //       // path to your AsyncAPI files
  //       pathToSpec: [path.join(__dirname, '../../lunar_frontiers/rover_projector.yaml')],

  //       // version events if already in catalog (optional)
  //       versionEvents: false
  //     },
  //   ]
  // ],
  secondaryCTA: {
    label: 'Getting Started',
    href:"https://www.eventcatalog.dev/"
  },
  logo: {
    alt: 'EventCatalog Logo',
    // found in the public dir
    src: 'logo.svg',
  },
  footerLinks: [
    { label: 'Events', href: '/events' },
    { label: 'Services', href: '/services' },
    { label: 'Visualiser', href: '/visualiser' },
    { label: '3D Node Graph', href: '/overview' },
    { label: 'GitHub', href: 'https://github.com/boyney123/eventcatalog-demo/edit/master' }
  ],
  users: [
    {
      id: 'dboyne',
      name: 'David Boyne',
      avatarUrl: 'https://pbs.twimg.com/profile_images/1262283153563140096/DYRDqKg6_400x400.png',
      role: 'Developer',
    },
    {
      id: 'mSmith',
      name: 'Matthew Smith',
      avatarUrl: 'https://randomuser.me/api/portraits/lego/3.jpg',
      role: 'Developer',
    },
  ],
}
