# html/dom/elements/global-attributes/the-anchor-attribute-004.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-anchor-attribute-004.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:masonf@chromium.org">
<link rel="help" href="https://github.com/whatwg/html/pull/9144">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=anchor1>Anchor 1</div>
<div id=anchor2>Anchor 2</div>
<svg id=foo anchor=anchor1></svg>

<script>
test(() => {
  assert_equals(foo.anchorElement,anchor1,'Non-HTML elements can use the anchor attribute');
  foo.anchorElement = anchor2;
  assert_equals(foo.anchorElement,anchor2,'The anchorElement IDL also works for non-HTML elements');
});
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
  "source_name": "html/dom/elements/global-attributes/the-anchor-attribute-004.tentative.html"
}
```
