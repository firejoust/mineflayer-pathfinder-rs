import { inject } from "./index.js"

const bot = {}
bot.world = {}
bot.world.columns = { hi: "test" }

inject(bot)
console.log(bot)