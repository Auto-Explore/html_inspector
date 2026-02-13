# html/rendering/non-replaced-elements/flow-content-0/search-styles-iso-8859-8.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/search-styles-iso-8859-8.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="ISO-8859-8">
<title>default styles for the search element, ISO-8859-8</title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/rendering/support/test-ua-stylesheet.js"></script>
<style>
/* Specify this bogus namespace, so the rules in this stylesheet only apply to the `fakeClone`d elements in #refs, not the HTML elements in #tests. */
@namespace url(urn:not-html);

address, blockquote, center, dialog, div, figure, figcaption, footer, form,
header, hr, legend, listing, main, p, plaintext, pre, search, xmp {
  display: block;
}

address, blockquote, center, div, figure, figcaption, footer, form, header, hr,
legend, listing, main, p, plaintext, pre, summary, xmp, article, aside, h1, h2,
h3, h4, h5, h6, hgroup, nav, section, search, table, caption, colgroup, col,
thead, tbody, tfoot, tr, td, th, dir, dd, dl, dt, menu, ol, ul, li, bdi, output,
[dir=ltr i], [dir=rtl i], [dir=auto i], * /* only apply to the bogus namespace */ {
  unicode-bidi: isolate;
}

</style>

<div id="log"></div>

<div id="tests">
  <search></search>
</div>

<div id="refs"></div>

<script>
  test(() => {
    assert_equals(document.characterSet, "ISO-8859-8")
  }, "Verify document.characterSet is ISO-8859-8");

  const props = [
    'display',
    'margin-top',
    'margin-right',
    'margin-bottom',
    'margin-left',
    'padding-top',
    'padding-right',
    'padding-bottom',
    'padding-left',
    'unicode-bidi',
  ];
  runUAStyleTests(props);

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “ISO-8859-8” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 43,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/search-styles-iso-8859-8.html"
}
```
