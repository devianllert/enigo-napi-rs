const {
  moveMouseRel,
  moveMouseAbs,
  mouseClick,
  mouseDown,
  mouseUp,
  mouseScroll,
} = require('./index')

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

const actions = {
  async rel() {
    console.log('moveMouseRel(+30, +30)')
    moveMouseRel(30, 30)
    await sleep(300)
    console.log('moveMouseRel(-30, -30)')
    moveMouseRel(-30, -30)
  },

  async abs() {
    const x = Number(process.env.TEST_ABS_X ?? 500)
    const y = Number(process.env.TEST_ABS_Y ?? 500)
    console.log(`moveMouseAbs(${x}, ${y})`)
    moveMouseAbs(x, y)
  },

  async click() {
    console.log("mouseClick('left')")
    mouseClick('left')
  },

  async downUp() {
    console.log("mouseDown('left')")
    mouseDown('left')
    await sleep(200)
    console.log("mouseUp('left')")
    mouseUp('left')
  },

  async scroll() {
    console.log('mouseScroll(3, true)  — scroll down')
    mouseScroll(3, true)
    await sleep(300)
    console.log('mouseScroll(-3, true) — scroll up')
    mouseScroll(-3, true)
  },

  async all() {
    await actions.rel()
    await sleep(300)
    await actions.click()
    await sleep(300)
    await actions.scroll()
  },
}

function printUsage() {
  console.log(`Usage: node simple-test.js [action]

Actions:
  all     run a short demo (default)
  rel     relative mouse move
  abs     absolute mouse move (TEST_ABS_X / TEST_ABS_Y env vars)
  click   left mouse click
  downUp  press and release left button
  scroll  vertical scroll down and up

Examples:
  node simple-test.js
  node simple-test.js click
  TEST_ABS_X=800 TEST_ABS_Y=400 node simple-test.js abs
`)
}

async function main() {
  const action = process.argv[2] ?? 'all'

  if (action === '--help' || action === '-h') {
    printUsage()
    return
  }

  if (!actions[action]) {
    console.error(`Unknown action: ${action}\n`)
    printUsage()
    process.exit(1)
  }

  console.log('enigo-napi-rs manual test')
  console.log(`Action: ${action}`)
  console.log('Starting in 3 seconds — focus a window to observe mouse input')
  await sleep(3000)

  await actions[action]()

  console.log('Done.')
}

main().catch((err) => {
  console.error('Test failed:', err)
  process.exit(1)
})
