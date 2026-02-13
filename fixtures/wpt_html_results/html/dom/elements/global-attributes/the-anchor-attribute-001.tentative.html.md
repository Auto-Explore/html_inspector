# html/dom/elements/global-attributes/the-anchor-attribute-001.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-anchor-attribute-001.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://github.com/whatwg/html/pull/9144">
<link rel="author" href="mailto:xiaochengh@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
body {
  margin: 0;
}
#anchor {
  width: 100px;
  height: 100px;
  margin-left: 50px;
  margin-top: 50px;
  background: orange;
}
#target {
  position: absolute;
  left: anchor(right);
  top: anchor(top);
  width: 100px;
  height: 100px;
  background: lime;
}
</style>
<div id="anchor"></div>
<div id="target" anchor="anchor"></div>

<script>
test(() => {
  assert_equals(target.offsetLeft, 150);
  assert_equals(target.offsetTop, 50);
}, 'The anchor attribute should position the target element next to its implicit anchor');

test(() => {
  assert_equals(target.anchorElement, anchor);
}, 'The element.anchorElement IDL should reflect the element pointed to by the anchor attribute');
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/dom/elements/global-attributes/the-anchor-attribute-001.tentative.html"
}
```
