module.exports = {
    devServer: {
        proxy: [
            {
                context: ['/graphql', '/login', '/logout', '/callback'],
                target: 'http://localhost:8080'
            }
        ],
        devMiddleware: {
            writeToDisk: true
        }
    },
};
