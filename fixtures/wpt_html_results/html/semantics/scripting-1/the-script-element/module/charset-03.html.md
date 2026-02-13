# html/semantics/scripting-1/the-script-element/module/charset-03.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/charset-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Imported module scripts should always use UTF-8</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script type="module" src="resources/import-utf8.js"></script>
<script type="module">
test(function() {
  assert_equals(window.getSomeString(), "śćążź",
                'Should be decoded as UTF-8');
}, 'UTF-8 imported module script');
</script>

<script type="module" src="resources/import-utf8-with-charset-header.js"></script>
<script type="module">
test(function() {
  assert_equals(window.getSomeString(), "śćążź",
                'Should be decoded as UTF-8');
}, 'UTF-8 imported module script with wrong charset in Content-Type');
</script>

<script type="module" src="resources/import-non-utf8.js"></script>
<script type="module">
test(function() {
  assert_not_equals(window.getSomeString(), "śćążź",
                    'Should be decoded as UTF-8');
}, 'Non-UTF-8 imported module script');
</script>

<script type="module" src="resources/import-non-utf8-with-charset-header.js"></script>
<script type="module">
test(function() {
  assert_not_equals(window.getSomeString(), "śćążź",
                    'Should be decoded as UTF-8');
}, 'Non-UTF-8 imported module script with charset in Content-Type');
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/charset-03.html"
}
```
