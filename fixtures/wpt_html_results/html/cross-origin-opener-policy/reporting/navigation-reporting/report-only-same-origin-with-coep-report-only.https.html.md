# html/cross-origin-opener-policy/reporting/navigation-reporting/report-only-same-origin-with-coep-report-only.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/navigation-reporting/report-only-same-origin-with-coep-report-only.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html

<meta name=timeout content=long>
<title>reporting same origin with report-to</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/cross-origin-opener-policy/resources/common.js"></script>
<script src="/html/cross-origin-opener-policy/reporting/resources/reporting-common.js"></script>

<script>

let tests = [
  // popup origin, popup COOP, popup COEP, popup COOP report-only, popup COEP report-only, expected reports

  // Open a cross-origin popup with COOP report-only with coep, which mismatches
  // with the current document (opener) COOP (unsafe-none) and COOP report-only
  // (same-origin) values.
  [
    SAME_ORIGIN,
    "",
    "require-corp",
    `same-origin; report-to="${popupReportOnlyEndpoint.name}"`,
    "",
    []
  ],
];

runNavigationReportingTests(document.title, tests);

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
        "byte_end": 33,
        "byte_start": 1,
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
  "source_name": "html/cross-origin-opener-policy/reporting/navigation-reporting/report-only-same-origin-with-coep-report-only.https.html"
}
```
