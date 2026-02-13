# html/semantics/the-button-element/command-and-commandfor/fullscreen-behavior.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/fullscreen-behavior.tentative.html",
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

<div id="invokee">
  Fullscreen content
  <button id="invokerbutton" commandfor="invokee"></button>
</div>

<script>
  async function resetState() {
    invokerbutton.setAttribute("command", "toggle-fullscreen");
    if (document.fullscreenElement) await document.exitFullscreen();
  }

  // toggle-fullscreen

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_false(invokee.matches(":fullscreen"));
    invokerbutton.setAttribute("command", "toggle-fullscreen");
    await clickOn(invokerbutton);
    assert_true(invokee.matches(":fullscreen"));
  }, "invoking div with toggle-fullscreen action makes div fullscreen");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_false(invokee.matches(":fullscreen"));
    invokerbutton.setAttribute("command", "toggle-fullscreen");
    invokerbutton.click();
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking div with toggle-fullscreen action (without user activation) is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    assert_false(invokee.matches(":fullscreen"));
    invokerbutton.setAttribute("command", "toggle-fullscreen");
    await clickOn(invokerbutton);
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking div with toggle-fullscreen action and preventDefault is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "toggle-fullscreen");
    await test_driver.bless('go fullscreen');
    await invokee.requestFullscreen();
    assert_true(invokee.matches(":fullscreen"));
    await clickOn(invokerbutton);
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking fullscreen div with toggle-fullscreen action exits fullscreen");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "tOgGlE-Fullscreen");
    await test_driver.bless('go fullscreen');
    await invokee.requestFullscreen();
    assert_true(invokee.matches(":fullscreen"));
    await clickOn(invokerbutton);
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking fullscreen div with toggle-fullscreen (case-insensitive) action exits fullscreen");

  // request-fullscreen

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_false(invokee.matches(":fullscreen"));
    invokerbutton.setAttribute("command", "request-fullscreen");
    await clickOn(invokerbutton);
    assert_true(invokee.matches(":fullscreen"));
  }, "invoking div with request-fullscreen action makes div fullscreen");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    assert_false(invokee.matches(":fullscreen"));
    invokerbutton.setAttribute("command", "request-fullscreen");
    await clickOn(invokerbutton);
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking div with request-fullscreen action and preventDefault is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "request-fullscreen");
    await test_driver.bless('go fullscreen');
    await invokee.requestFullscreen();
    assert_true(invokee.matches(":fullscreen"));
    await clickOn(invokerbutton);
    assert_true(invokee.matches(":fullscreen"));
  }, "invoking fullscreen div with request-fullscreen action is a no-op");

  // exitFullscreen

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    assert_false(invokee.matches(":fullscreen"));
    invokerbutton.setAttribute("command", "exit-fullscreen");
    await clickOn(invokerbutton);
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking div with exit-fullscreen action is a no-op");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "exit-fullscreen");
    await test_driver.bless('go fullscreen');
    await invokee.requestFullscreen();
    assert_true(invokee.matches(":fullscreen"));
    await clickOn(invokerbutton);
    assert_false(invokee.matches(":fullscreen"));
  }, "invoking fullscreen div with exit-fullscreen action exits fullscreen");

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    invokerbutton.setAttribute("command", "exit-fullscreen");
    await test_driver.bless('go fullscreen');
    await invokee.requestFullscreen();
    assert_true(invokee.matches(":fullscreen"));
    await clickOn(invokerbutton);
    assert_true(invokee.matches(":fullscreen"));
  }, "invoking fullscreen div with exit-fullscreen action and preventDefault is a no-op");

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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/fullscreen-behavior.tentative.html"
}
```
