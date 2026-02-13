# html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-all.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-all.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.all</title>
<link rel="author" title="Corey Farwell" href="mailto:coreyf@rwell.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/obsolete.html#dom-document-all">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function () {
  assert_false(Boolean(document.all));

  assert_true(document.all == undefined);
  assert_true(document.all == null);
  assert_false(document.all != undefined);
  assert_false(document.all != null);

  assert_true(document.all !== undefined);
  assert_true(document.all !== null);
  assert_false(document.all === undefined);
  assert_false(document.all === null);

  assert_equals(typeof document.all, "undefined");

  if (document.all) { assert_true(false); }

  if (!document.all) {}
  else { assert_true(false); }
}, "'unusual behaviors' of document.all")

test(function() {
  var all = document.all;

  assert_false(Boolean(all));

  assert_true(all == undefined);
  assert_true(all == null);
  assert_false(all != undefined);
  assert_false(all != null);

  assert_true(all !== undefined);
  assert_true(all !== null);
  assert_false(all === undefined);
  assert_false(all === null);

  assert_equals(typeof all, "undefined");

  if (all) { assert_true(false); }

  if (!all) {}
  else { assert_true(false); }
}, "'unusual behaviors' of document.all with assignment")
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
  "source_name": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-all.html"
}
```
