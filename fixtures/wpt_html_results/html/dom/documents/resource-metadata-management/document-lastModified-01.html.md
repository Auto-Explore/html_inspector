# html/dom/documents/resource-metadata-management/document-lastModified-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/resource-metadata-management/document-lastModified-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.lastModified should return current local time</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/document-lastModified-utils.js"></script>
<div id="log"></div>
<script>
  var last_modified = document.lastModified;

  test(function() {
     assert_regexp_match(last_modified, DOCUMENT_LASTMODIFIED_REGEX,
                         "Format should match the pattern \"NN/NN/NNNN NN:NN:NN\".");
  }, "Date returned by lastModified is in the form \"MM/DD/YYYY hh:mm:ss\".");

  test(function() {
    assert_document_lastmodified_string_approximately_now(last_modified);
  }, "Date returned by lastModified is current at page load");

  var t = async_test("Date returned by lastModified is current after timeout.");
  t.step_timeout(function() {
    assert_document_lastmodified_string_approximately_now(document.lastModified);
    t.done();
  }, 4000);
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
  "source_name": "html/dom/documents/resource-metadata-management/document-lastModified-01.html"
}
```
