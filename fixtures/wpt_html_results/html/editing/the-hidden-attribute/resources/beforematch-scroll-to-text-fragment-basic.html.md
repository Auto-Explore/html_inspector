# html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-basic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-basic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/scroll-to-text-fragment/stash.js"></script>

<!-- This test is navigated to with the fragment #:~:text=foo -->

<div style="height: 4000px;">spacer</div>

<script>
(async () => {
  const results = {};
  const key = (new URLSearchParams(window.location.search)).get('key');

  // This test adds the elements from JS
  // (unlike beforematch-scroll-to-text-fragment-with-anchor.html,
  // which uses parser-created elements)
  const foo = document.createElement('div');
  foo.hidden = 'until-found';
  foo.textContent = 'foo';
  document.body.appendChild(foo);

  let beforematchResolver = null;
  const beforematchPromise = new Promise(resolve => {
    beforematchResolver = resolve;
  });

  // This should be true. Foo was searched for, so it should get a
  // beforematch event.
  results.beforematchFiredOnFoo = false;
  // This should be true. the hidden attribute should not be removed until
  // after beforematch is fired.
  results.beforematchHiddenAttributePresent = false;
  foo.addEventListener('beforematch', () => {
    results.beforematchFiredOnFoo = true;
    results.beforematchHiddenAttributePresent = foo.hasAttribute('hidden');
    // This should be zero. Scrolling should happen after beforematch is
    // fired.
    results.pageYOffsetDuringBeforematch = window.pageYOffset;
    beforematchResolver();
  });

  const bar = document.createElement('div');
  bar.hidden = 'until-found';
  bar.textContent = 'bar';
  document.body.appendChild(bar);
  // This should be false. Bar was not searched for, so it should not get
  // a beforematch event.
  results.beforematchFiredOnBar = false;
  bar.addEventListener('beforematch', () => {
    // this handler should never run. If it does,
    // send back the message immediately to make the test fail.
    results.beforematchFiredOnBar = true;
    stashResultsThenClose(key, results);
  });

  await beforematchPromise;
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);
  // This should be greater than zero. The page should be scrolled down
  // to foo.
  results.pageYOffsetAfterRaf = window.pageYOffset;
  stashResultsThenClose(key, results);
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
  "source_name": "html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-basic.html"
}
```
