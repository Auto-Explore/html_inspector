# html/editing/the-hidden-attribute/resources/hidden-until-found-text-fragment.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/resources/hidden-until-found-text-fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/scroll-to-text-fragment/stash.js"></script>

<!-- This test is navigated to with the fragment #:~:text=foo -->

<body>
  <div style="height: 4000px;">spacer</div>
  <div id=target hidden=until-found>foo</div>
  <script>
    // scroll-to-text-fragment may delay scrolling for an
    // arbitrary amount of time for security reasons.
    // This test would time out if beforematch is not fired, but since the
    // test file only has one test, this is not shadowing other test results.
    target.onbeforematch = () => {
      // Adding two additional requestAnimationFrames ensures
      // that scrolling has happened after beforematch has fired.
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          const results = {};
          // This should be false. The hidden=until-found attribute should be
          // removed in response to ScrollToTextFragment.
          results.targetHasHiddenAttribute = document.getElementById('target').hasAttribute('hidden');
          // This should be greater than zero. The page should be scrolled down
          // to foo.
          results.pageYOffsetAfterRaf = window.pageYOffset;

          params = new URLSearchParams(window.location.search);
          stashResultsThenClose(params.get('key'), results);

        });
      });
    };
  </script>
</body>
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
  "source_name": "html/editing/the-hidden-attribute/resources/hidden-until-found-text-fragment.html"
}
```
