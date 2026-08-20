'use strict'

module.exports = function plopfile(plop) {
    plop.setGenerator('component', require('./generators/component/index.cjs'))
    plop.setGenerator('page', require('./generators/page/index.cjs'))
    plop.setGenerator('api', require('./generators/api/index.cjs'))
    plop.setGenerator('hook', require('./generators/hook/index.cjs'))
    plop.setGenerator('store', require('./generators/store/index.cjs'))
}
