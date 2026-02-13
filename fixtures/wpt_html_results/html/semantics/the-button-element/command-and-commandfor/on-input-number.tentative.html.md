# html/semantics/the-button-element/command-and-commandfor/on-input-number.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-input-number.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Luke Warlow" href="mailto:lwarlow@igalia.com" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<input type="number" id="invokee" value="0">
<button id="invokerbutton" commandfor="invokee"></button>

<script>
  function reset() {
    invokee.value = 0;
    invokerbutton.removeAttribute('command');
  }

  // step-up

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_equals(invokee.valueAsNumber, 0);
    invokerbutton.setAttribute("command", "step-up");
    await clickOn(invokerbutton);
    assert_equals(invokee.valueAsNumber, 1);
  }, "invoking number input with step-up action increments value");

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_equals(invokee.valueAsNumber, 0);
    invokerbutton.setAttribute("command", "sTeP-uP");
    await clickOn(invokerbutton);
    assert_equals(invokee.valueAsNumber, 1);
  }, "invoking number input with step-up action (case-insensitive) increments value");

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_equals(invokee.valueAsNumber, 0);
    invokerbutton.setAttribute("command", "step-up");
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    await clickOn(invokerbutton);
    assert_equals(invokee.valueAsNumber, 0);
  }, "invoking number input with step-up action and preventDefault does not increment value");

  // step-down

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_equals(invokee.valueAsNumber, 0);
    invokerbutton.setAttribute("command", "step-down");
    await clickOn(invokerbutton);
    assert_equals(invokee.valueAsNumber, -1);
  }, "invoking number input with step-down action decrements value");

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_equals(invokee.valueAsNumber, 0);
    invokerbutton.setAttribute("command", "sTeP-dOwN");
    await clickOn(invokerbutton);
    assert_equals(invokee.valueAsNumber, -1);
  }, "invoking number input with step-down action (case-insensitive) decrements value");

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_equals(invokee.valueAsNumber, 0);
    invokerbutton.setAttribute("command", "step-down");
    invokee.addEventListener("command", (e) => e.preventDefault(), {
      once: true,
    });
    await clickOn(invokerbutton);
    assert_equals(invokee.valueAsNumber, 0);
  }, "invoking number input with step-down action and preventDefault does not decrement value");
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
        "byte_end": 116,
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
        "byte_end": 116,
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-input-number.tentative.html"
}
```
