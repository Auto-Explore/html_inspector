# html/editing/the-hidden-attribute/beforematch-infinite-loop.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/beforematch-infinite-loop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/#ancestor-revealing-algorithm">
<link rel=help href="https://github.com/whatwg/html/issues/11436">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=h1 hidden=until-found>
  <div id=h1child>hidden 1</div>
</div>
<div id=h2 hidden=until-found>hidden 2</div>

<script>
test(() => {
  let beforematchFired1 = false;
  let beforematchFired2 = false;
  h1.addEventListener('beforematch', () => {
    beforematchFired1 = true;
    h2.appendChild(h1);
  });
  h2.addEventListener('beforematch', () => {
    beforematchFired2 = true;
  });

  window.location.hash = '#h1child';
  assert_true(beforematchFired1, 'beforematch should have been fired on h1.');
  assert_false(beforematchFired2, 'beforematch should not have been fired on h2.');
  assert_false(h1.hasAttribute('hidden'), 'h1 should not be hidden.');
  assert_true(h2.hasAttribute('hidden'), 'h2 should be hidden.');
}, 'hidden=until-found revealing algorithm should collect elements to reveal before revealing them.');
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
  "source_name": "html/editing/the-hidden-attribute/beforematch-infinite-loop.html"
}
```
