# html/dom/elements/global-attributes/the-anchor-attribute-002.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-anchor-attribute-002.tentative.html",
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
.anchor {
  width: 100px;
  height: 100px;
  margin-left: 50px;
  margin-top: 50px;
  background: orange;
}
.target {
  position: absolute;
  left: anchor(right, 123px);
  top: anchor(top, 456px);
  width: 100px;
  height: 100px;
  background: lime;
}
</style>
<div class="anchor" id="anchor1"></div>
<div class="anchor" id="anchor2"></div>
<div class="target" id="target1" anchor="anchor1"></div>
<div class="target" id="target2" anchor="anchor1"></div>

<script>
test(() => {
  document.body.offsetLeft; // Force layout
  target1.setAttribute('anchor', 'anchor2');
  assert_equals(target1.offsetLeft, 150);
  assert_equals(target1.offsetTop, 200);

  target1.setAttribute('anchor', 'anchor1');
  assert_equals(target1.offsetLeft, 150);
  assert_equals(target1.offsetTop, 50);
}, 'Layout should be updated when anchor attribute changes to another element');

test(() => {
  document.body.offsetLeft; // Force layout
  target2.setAttribute('anchor', 'nonexist-anchor');
  assert_equals(target2.offsetLeft, 123);
  assert_equals(target2.offsetTop, 456);
}, 'Layout should be updated when anchor attribute changes to a non-existent element');
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
  "source_name": "html/dom/elements/global-attributes/the-anchor-attribute-002.tentative.html"
}
```
