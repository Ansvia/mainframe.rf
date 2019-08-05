const path = require('path')
module.exports = {
  productionSourceMap: false,
  pluginOptions: {
    'style-resources-loader': {
      patterns: [
        path.resolve(__dirname, 'src/assets/less/variable.less'),
        path.resolve(__dirname, 'src/assets/less/mixins.less'),
      ],
      preProcessor: 'less'
    }
  },
  runtimeCompiler: true,
  pwa: {
    themeColor: '#232731',    
    workboxOptions: {
      skipWaiting: true,
      clientsClaim: true,
    }  
  }
}
