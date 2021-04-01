//注意，这里需要一个动态的' import '语句，因为
// webpack/webpack#6615，但理论上`import {greet} from ` ./pkg `;
//有一天也会在这里工作!

const rust = import('./pkg');

rust
    .then(m => m.greet('绣儿'))
    .catch(console.error);