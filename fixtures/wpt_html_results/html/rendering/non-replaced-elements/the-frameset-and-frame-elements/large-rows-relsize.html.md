# html/rendering/non-replaced-elements/the-frameset-and-frame-elements/large-rows-relsize.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/large-rows-relsize.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="author" title="Morten Stenshorne" href="mailto:mstensho@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#frames-and-framesets">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1116832">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1353277">
<script>
window.onload = () => {
  test(() => {
    const frameSet = document.querySelector('frameset');
    const frames = document.querySelectorAll('frame');
    assert_less_than(frames[0].offsetHeight, frameSet.offsetHeight);
    assert_greater_than(frames[0].offsetHeight, frames[1].offsetHeight);
    assert_greater_than_equal(frames[1].offsetHeight, 0);
  }, 'A large relative value should not produce weird sizes.');
};
</script>
<frameset rows="4294967227*,*" frameborder="0">
  <frame src="resources/green.html">
  <frame src="resources/red.html">
</frameset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 962,
        "byte_start": 915,
        "col": 1,
        "line": 19
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
  "source_name": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/large-rows-relsize.html"
}
```
