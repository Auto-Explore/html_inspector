# html/cross-origin-opener-policy/reporting/access-reporting/property-postmessage-1.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/access-reporting/property-postmessage-1.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title> Check openee.postMessage(arg1, arg2) access is checked</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/get-host-info.sub.js></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/cross-origin-opener-policy/reporting/resources/reporting-common.js"></script>
<script src="/html/cross-origin-opener-policy/reporting/resources/test-access-property.js"></script>
<script>

testAccessProperty("postMessage", w => w.postMessage("", ""));

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/cross-origin-opener-policy/reporting/access-reporting/property-postmessage-1.https.html"
}
```
