# html/semantics/forms/the-select-element/select-many-options.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-many-options.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>HTMLselectElement Test: many options</title>
<link rel="author" title="Ionel Popescu" href="mailto:iopopesc@microsoft.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
  #select0 {
    position: absolute;
    top: 0px;
    left: 0px;
  }

  ::picker(select) {
    border: 1px solid rgba(0, 0, 0, 0.15);
    border-radius: 4px;
    box-shadow: 0px 12.8px 28.8px rgba(0, 0, 0, 0.13), 0px 0px 9.2px rgba(0, 0, 0, 0.11);
    box-sizing: border-box;
    overflow: auto;
    padding: 4px;
  }
  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<select id="select0">
  <div id="select0-popover">
    <option>bottom left</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
    <option>two</option>
    <option>three</option>
  </div>
</select>
<br>

<script>
  function clickOn(element) {
    const actions = new test_driver.Actions();
    return actions.pointerMove(0, 0, {origin: element})
      .pointerDown({button: actions.ButtonType.LEFT})
      .pointerUp({button: actions.ButtonType.LEFT})
      .send();
  }

  promise_test(async () => {
    const select0 = document.getElementById("select0");
    const select0Popover = document.getElementById("select0-popover");

    await clickOn(select0);
    assert_equals(Math.round(select0.getBoundingClientRect().bottom), Math.round(select0Popover.getBoundingClientRect().top));
    assert_equals(Math.round(select0.getBoundingClientRect().left), Math.round(select0Popover.getBoundingClientRect().left));
    assert_equals(window.innerHeight, Math.round(select0Popover.getBoundingClientRect().bottom));
  }, "The popover should be bottom left positioned");

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
  "source_name": "html/semantics/forms/the-select-element/select-many-options.tentative.html"
}
```
