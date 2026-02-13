# html/semantics/interactive-elements/the-details-element/details-add-summary.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/details-add-summary.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<link rel=match href="details-add-summary-ref.html">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/6466">

<!-- This test makes sure that new <summary> elements get rendered correctly
  when added to a <details> element. I ran into it when adding
  content-visibility:hidden to the second slot of <details>. -->

<script>
onload = () => {
  const newsummary = document.createElement('summary');
  newsummary.textContent = 'new summary';
  document.getElementById('detailsid').insertBefore(newsummary,
    document.getElementById('oldsummary'));

  document.documentElement.classList.remove('reftest-wait');
};
</script>

<details id=detailsid>
  <summary id=oldsummary>old summary</summary>
  details
</details>
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
  "source_name": "html/semantics/interactive-elements/the-details-element/details-add-summary.html"
}
```
