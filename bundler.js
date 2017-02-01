var Webpack = require('webpack');
var WebpackDevServer = require('webpack-dev-server');
var webpackConfig = require('./webpack.config.js');
var host = process.env.APP_HOST || 'localhost';

module.exports = function() {
    var bundleStart = null;
    var compiler = Webpack(webpackConfig);

    compiler.plugin('compile', function() {
        console.log('Bundling...');
        bundleStart = Date.now();
    });

    compiler.plugin('done', function() {
        console.log('Bundled in ' + (Date.now() - bundleStart) + 'ms!');
    });

    var bundler = new WebpackDevServer(compiler, {
        publicPath: '/assets/',
        hot: true,

        quiet: false,
        noInfo: true,
        stats: {
            colors: true
        }
    });

    bundler.listen(3001, host, function() {
        console.log('Bundling project, please wait...');
    });
};
