const {
  moveMouseRel,
  moveMouseAbs,
  mouseClick,
  mouseDown,
  mouseUp,
  mouseScroll,
  keyTap,
  keyDown,
  keyUp,
  typeText,
} = require('./index')

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

const actions = {
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
    console.log('--- mouseScroll(length, isVertical) — arbitrary wheel amount ---')
    console.log('mouseScroll(3, true)  — 3 ticks down')
    mouseScroll(3, true)
    await sleep(300)
    console.log('mouseScroll(-3, true) — 3 ticks up')
    mouseScroll(-3, true)
    await sleep(300)
    console.log('mouseScroll(2, false) — 2 ticks right')
    mouseScroll(2, false)
    await sleep(300)
    console.log('mouseScroll(-2, false) — 2 ticks left')
    mouseScroll(-2, false)
  },

  async scrollClick() {
    console.log('--- mouseClick(scroll*) — one wheel tick per click ---')
    console.log("mouseClick('scrollDown')")
    mouseClick('scrollDown')
    await sleep(300)
    console.log("mouseClick('scrollUp')")
    mouseClick('scrollUp')
    await sleep(300)
    console.log("mouseClick('scrollRight')")
    mouseClick('scrollRight')
    await sleep(300)
    console.log("mouseClick('scrollLeft')")
    mouseClick('scrollLeft')
  },

  async middle() {
    console.log("mouseClick('middle')")
    await sleep(300)
    mouseClick('middle')
    await sleep(300)
    mouseClick('middle')
  },

  async rel() {
    console.log('moveMouseRel(+30, +30)')
    moveMouseRel(30, 30)
    await sleep(300)
    console.log('moveMouseRel(-30, -30)')
    moveMouseRel(-30, -30)
  },

  async keys() {
    console.log('--- keyTap — layout-independent physical keys ---')
    console.log("keyTap('f')")
    keyTap('f')
    await sleep(200)
    console.log("keyTap('a')")
    keyTap('a')
    await sleep(200)
    console.log("keyDown('shift') + keyTap('f') + keyUp('shift')")
    keyDown('shift')
    await sleep(100)
    keyTap('f')
    await sleep(100)
    keyUp('shift')
    await sleep(200)
    console.log("keyTap('enter')")
    keyTap('enter')
    await sleep(200)
    console.log("keyTap('f1')")
    keyTap('f1')
  },

  async text() {
    const sample = process.argv.slice(3).join(' ') || '/hideout'
    console.log('--- typeText — Unicode input (layout-independent) ---')
    console.log(`typeText('${sample}')`)
    typeText(sample)
  },

  async all() {
    await actions.rel()
    await sleep(300)
    await actions.click()
    await sleep(300)
    await actions.middle()
    await sleep(300)
    await actions.scroll()
    await sleep(500)
    await actions.scrollClick()
    await sleep(500)
    await actions.keys()
  },
}

function printUsage() {
  console.log(`Usage: node simple-test.js [action] [args...]

Actions:
  all          run full demo (default)
  rel          relative mouse move
  abs          absolute mouse move (TEST_ABS_X / TEST_ABS_Y env vars)
  click        left mouse click
  middle       middle mouse button click
  downUp       press and release left button
  scroll       scroll via mouseScroll(length, isVertical)
  scrollClick  scroll via mouseClick('scrollDown' | 'scrollUp' | ...)
  keys         keyboard: f, a, shift+f, enter, f1
  text         type a string via typeText (default: /hideout)

Examples:
  node simple-test.js
  node simple-test.js text /hideout
  node simple-test.js text "hello world"
  node simple-test.js keys
  node simple-test.js scrollClick
  node simple-test.js scroll
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
  if (action === 'keys' || action === 'text' || action === 'all') {
    console.log('Starting in 3 seconds — focus a text field (Notepad, browser input, etc.)')
  } else {
    console.log('Starting in 3 seconds — focus a window to observe mouse input')
  }
  await sleep(3000)

  await actions[action]()

  console.log('Done.')
}

main().catch((err) => {
  console.error('Test failed:', err)
  process.exit(1)
})
