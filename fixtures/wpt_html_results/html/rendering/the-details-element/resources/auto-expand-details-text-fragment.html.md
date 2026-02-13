# html/rendering/the-details-element/resources/auto-expand-details-text-fragment.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/resources/auto-expand-details-text-fragment.html",
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
  <details>
    <div>foo</div>
  </details>
  <script>
    const details = document.querySelector("details");
    details.ontoggle = () => {
      const results = {};
      // This should be true. The details element should be opened by
      // ScrollToTextFragment because it has matching text.
      results.detailsHasOpenAttribute = document.querySelector('details').hasAttribute('open');
      // This should be greater than zero. The page should be scrolled down
      // to the matching target.
      results.pageYOffsetAfterRaf = window.pageYOffset;

      params = new URLSearchParams(window.location.search);
      stashResultsThenClose(params.get('key'), results);
    };
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 236,
        "byte_start": 226,
        "col": 3,
        "line": 10
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
  "source_name": "html/rendering/the-details-element/resources/auto-expand-details-text-fragment.html"
}
```
