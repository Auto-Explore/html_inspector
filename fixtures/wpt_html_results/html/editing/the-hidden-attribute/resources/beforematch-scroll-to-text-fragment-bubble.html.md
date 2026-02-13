# html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-bubble.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-bubble.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/scroll-to-text-fragment/stash.js"></script>

<!-- This test is navigated to with the fragment #:~:text=foo -->

<div id=parentid>
  <div id=childid hidden=until-found>foo</div>
</div>

<script>
(async () => {
  const results = {
    beforematchFiredOnChild: false,
    beforematchFiredOnParent: false
  };

  let beforematchResolver = null;
  const beforematchPromise = new Promise(resolve => {
    beforematchResolver = resolve;
  });

  childid.addEventListener('beforematch', () => {
    results.beforematchFiredOnChild = true;
    beforematchResolver();
  });

  parentid.addEventListener('beforematch', () => {
    results.beforematchFiredOnParent = true;
  });

  await beforematchPromise;
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);
  const params = new URLSearchParams(window.location.search);
  stashResultsThenClose(params.get('key'), results);
})();
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
  "source_name": "html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-bubble.html"
}
```
