# html/semantics/the-button-element/command-and-commandfor/on-audio-behavior.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-audio-behavior.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Luke Warlow" href="mailto:luke@warlow.dev" />
<meta name="timeout" content="long" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<audio controls id="invokee" src="/media/sound_5.mp3"></audio>
<button id="invokerbutton" commandfor="invokee" command="mute"></button>

<script>
  function resetState() {
    invokerbutton.setAttribute("command", "mute");
    invokee.pause();
    invokee.currentTime = 0;
    invokee.muted = false;
  }

  // play-pause

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "play-pause");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.paused);
  }, "invoking audio with play-pause action makes audio play");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "play-pause");
    invokerbutton.click();
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.paused);
  }, "invoking audio with play-pause action (without user activation) is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "play-pause");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.paused);
  }, "invoking audio with play-pause action and preventDefault is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    await test_driver.bless("play audio");
    invokee.play();
    assert_false(invokee.paused);
    invokerbutton.setAttribute("command", "play-pause");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.paused);
  }, "invoking playing audio with play-pause action pauses it");

  // play

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "play");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.paused);
  }, "invoking audio with play action makes audio play");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "play");
    invokerbutton.click();
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.paused);
  }, "invoking audio with play action (without user activation) is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "play");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.paused);
  }, "invoking audio with play action and preventDefault is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    await test_driver.bless("play audio");
    invokee.play();
    assert_false(invokee.paused);
    invokerbutton.setAttribute("command", "play");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.paused);
  }, "invoking playing audio with play action is a no-op");

  // pause

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "pause");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.paused);
  }, "invoking audio with pause action is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    assert_true(invokee.paused);
    invokerbutton.setAttribute("command", "pause");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.paused);
  }, "invoking audio with pause action and preventDefault is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    await test_driver.bless("play audio");
    invokee.play();
    assert_false(invokee.paused);
    invokerbutton.setAttribute("command", "pause");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.paused);
  }, "invoking playing audio with pause action makes it pause");

  // mute

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_false(invokee.muted);
    invokerbutton.setAttribute("command", "toggle-muted");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_true(invokee.muted);
  }, "invoking audio with toggle-muted action mutes it");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    assert_false(invokee.muted);
    invokerbutton.setAttribute("command", "toggle-muted");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.muted);
  }, "invoking audio with toggle-muted action and preventDefault is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.muted = true;
    assert_true(invokee.muted);
    invokerbutton.setAttribute("command", "toggle-muted");
    await clickOn(invokerbutton);
    await new Promise((resolve) => {
      requestAnimationFrame(resolve);
    });
    assert_false(invokee.muted);
  }, "invoking muted audio with toggle-muted action unmutes it");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 113,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 113,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-audio-behavior.tentative.html"
}
```
