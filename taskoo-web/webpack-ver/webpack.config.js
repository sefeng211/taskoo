const path = require('path');
var HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    static: './dist',
    port: 4141,
    allowedHosts: ['.seanfeng.dev'],
    headers : {
      'X-Forwarded-Proto': 'https'
    },
    proxy: {
      '/api': {
        target: 'http://100.86.23.103:4141',
        changeOrigin: true, // If needed, change the origin of the host header
        secure: false // If you're proxying to HTTP, ensure this is set to false
      }
    }
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
};
