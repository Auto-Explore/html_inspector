# html/rendering/the-details-element/details-display.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-display.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset=UTF-8>
<title>CSS Test: default display of details and summary elements</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<link rel="help" href="https://github.com/whatwg/html/pull/10265">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<details id="a">
  <summary id="b">This is the real summary.</summary>

  <p>This is the rest of the details.</p>
  <summary id="c">This summary is not special.</summary>
</details>

<summary id="d">This summary is not special.</summary>

<script>

  test(() => {
    assert_equals(getComputedStyle(document.getElementById("a")).display, "block");
  }, "default display of details element is block");
  test(() => {
    assert_equals(getComputedStyle(document.getElementById("b")).display, "list-item");
  }, "default display of first summary child of details is list-item");
  test(() => {
    assert_equals(getComputedStyle(document.getElementById("c")).display, "block");
  }, "default display of other summary element in details is block");
  test(() => {
    assert_equals(getComputedStyle(document.getElementById("d")).display, "block");
  }, "default display of summary element outside details is block");

  let new_styles = `
    details { display: grid }
    summary { display: flex }
  `;
  let new_style_element = document.createElement("style");
  new_style_element.append(document.createTextNode(new_styles));
  document.head.append(new_style_element);

  test(() => {
    assert_equals(getComputedStyle(document.getElementById("a")).display, "grid");
  }, "display of details element can be changed");
  test(() => {
    assert_equals(getComputedStyle(document.getElementById("b")).display, "flex");
  }, "display of first summary child of details can be changed");
  test(() => {
    assert_equals(getComputedStyle(document.getElementById("c")).display, "flex");
  }, "display of other summary element in details can be changed");
  test(() => {
    assert_equals(getComputedStyle(document.getElementById("d")).display, "flex");
  }, "display of summary element outside details can be changed");


</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.multiple_summary",
      "message": "Element “summary” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 524,
        "byte_start": 508,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/the-details-element/details-display.html"
}
```
