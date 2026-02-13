# html/editing/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: tabindex - focus, click</title>
<link rel="author" title="Intel" href="www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#the-tabindex-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<h2>Test steps</h2>
<p>Focus on the button below by "Tab" key, then press "Enter" key</p>

<p><button type="button">Test tabIndex</button></p>

<script>

setup({explicit_done: true});
setup({explicit_timeout: true});

promise_test(async t => {
  const button = document.querySelector("button");
  on_event(button, "click", () => {
    test(() => {
      assert_true(document.activeElement == button, "Focus on the button by Tab key");
    }, "Check if click event will be fired when press the 'enter' key while the element is focused");
    done();
  });

  window.focus();
  document.activeElement?.blur();
  getSelection().collapse(document.querySelector("p"), 0);
  const enterKey = '\uE007';
  await test_driver.send_keys(button, enterKey);
});

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-event.html"
}
```
