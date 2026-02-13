# html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-with-anchor.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-with-anchor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/scroll-to-text-fragment/stash.js"></script>

<!-- This test is navigated to with the fragment #bar:~:text=foo -->

<!-- This test uses parser-created elements,
  unlike beforematch-scroll-to-text-fragment-basic.html,
  which adds them from JS -->
<div style="height:10000px"></div>
<div id=foo hidden=until-found>foo</div>
<div id=bar hidden=until-found>bar</div>

<script>
(async () => {
  const results = {};
  const key = (new URLSearchParams(window.location.search)).get('key');

  // This should be true. hidden=until-found revealing should not happen until
  // after the script tag loads.
  results.fooHasHiddenAttribute = foo.hasAttribute('hidden');

  let beforematchResolver = null;
  const beforematchPromise = new Promise(resolve => {
    beforematchResolver = resolve;
  });

  // This should be true. Foo was searched for, so it should get the
  // beforematch event.
  results.beforematchFiredOnFoo = false;
  foo.addEventListener('beforematch', () => {
    results.beforematchFiredOnFoo = true;
    // This should be zero. Scrolling should happen after beforematch is fired.
    results.pageYOffsetDuringBeforematch = window.pageYOffset;
    beforematchResolver();
  });

  // This should be false. Bar should not get the beforematch event
  // despite being the target of an element fragment due to the text
  // fragment.
  results.beforematchFiredOnBar = false;
  bar.addEventListener('beforematch', () => {
    // this handler should never run. If it does,
    // send back the message immediately to make the test fail.
    results.beforematchFiredOnBar = true;
    stashResultsThenClose(key, results);
  });

  if (!results.fooHasHiddenAttribute) {
    // No need to wait for the beforematch event if it will never come.
    stashResultsThenClose(key, results);
  } else {
    await beforematchPromise;
    await new Promise(requestAnimationFrame);
    await new Promise(requestAnimationFrame);
    // This should be greater than zero. Scrolling should happen after the
    // beforematch event handler.
    results.pageYOffsetAfterRaf = window.pageYOffset;
    stashResultsThenClose(key, results);
  }
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
  "source_name": "html/editing/the-hidden-attribute/resources/beforematch-scroll-to-text-fragment-with-anchor.html"
}
```
