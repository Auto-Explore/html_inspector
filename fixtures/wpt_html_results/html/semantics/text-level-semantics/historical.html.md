# html/semantics/text-level-semantics/historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical text-level element features should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
function t(property, tagNames) {
  if (typeof tagNames === "string") {
    tagNames = [tagNames];
  }
  tagNames.forEach(function(tagName) {
    test(function() {
      assert_false(property in document.createElement(tagName));
    }, tagName + '.' + property + ' should not be supported');
  });
}

// <area> and <link> are in other sections in the spec, but we'll test them here together with <a>

// removed in https://github.com/whatwg/html/commit/790479ab1ba143efc27d1f92cd0465627df48fb0
t('hreflang', 'area');
t('type', 'area');

// renamed to dateTime in https://github.com/whatwg/html/commit/8b6732237c7021cd61e3c3463146234ca8ce5bad
t('datetime', 'time');

// removed in https://github.com/whatwg/html/commit/66fcb2357f205448fe2f40d7834a1e8ea2ed283b
t('media', ['a', 'area']);
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/text-level-semantics/historical.html"
}
```
