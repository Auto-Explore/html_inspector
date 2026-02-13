# html/browsers/browsing-the-web/scroll-to-fragid/target-pseudo-after-reinsertion.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/target-pseudo-after-reinsertion.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1252507">
<link rel=help href="https://html.spec.whatwg.org/multipage/browsing-the-web.html#target-element">
<link rel=help href="https://github.com/whatwg/html/issues/10029">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=target>target</div>

<script>
test(() => {
  const target = document.getElementById('target');
  window.location.href = '#target';
  assert_equals(document.querySelector(':target'), target,
    ':target should match before reinsertion.');

  target.remove();
  document.body.appendChild(target);
  assert_equals(document.querySelector(':target'), target,
    ':target should match after reinsertion.');
}, ':target should match the target element even after it is removed and reinserted.');
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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/target-pseudo-after-reinsertion.html"
}
```
