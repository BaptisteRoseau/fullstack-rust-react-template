const componentGenerator = require('./generators/component/index')
const pageGenerator = require('./generators/page/index')
const layoutGenerator = require('./generators/layout/index')
const featureGenerator = require('./generators/feature/index')
const hookGenerator = require('./generators/hook/index')
const apiGenerator = require('./generators/api/index')
const formGenerator = require('./generators/form/index')
const storeGenerator = require('./generators/store/index')

/**
 *
 * @param {import('plop').NodePlopAPI} plop
 */
module.exports = function (plop) {
    plop.setGenerator('component', componentGenerator)
    plop.setGenerator('page', pageGenerator)
    plop.setGenerator('layout', layoutGenerator)
    plop.setGenerator('feature', featureGenerator)
    plop.setGenerator('hook', hookGenerator)
    plop.setGenerator('api', apiGenerator)
    plop.setGenerator('form', formGenerator)
    plop.setGenerator('store', storeGenerator)
}
