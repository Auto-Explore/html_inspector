# html/cross-origin-opener-policy/reporting/navigation-reporting/report-only-same-origin-report-to.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/navigation-reporting/report-only-same-origin-report-to.https.html",
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
<script
  src="/html/cross-origin-opener-policy/reporting/resources/reporting-common.js?pipe=sub&report_id=380ca360-d1ae-4329-b1dd-69cea49cd705&report_only_id=cf9ac91d-6c5d-4489-a420-10be9402ef84"></script>

<script>

let tests = [
  // popup origin, popup COOP, popup COEP, popup COOP report-only, popup COEP report-only, expected reports

  // Open a cross-origin popup without any COOP setup, the current document
  // (opener) report-only would cause a browsing context group swap, hence a
  // report is sent to the corresponding endpoint.
  [
    CROSS_ORIGIN,
    "",
    "",
    "",
    "",
    [
      {
        "endpoint": reportOnlyEndpoint,
        "report": {
          "body": {
            "disposition": "reporting",
            "effectivePolicy": "same-origin",
            "nextResponseURL": /uuid=EXECUTOR_UUID$/, // next document URL
            "type": "navigation-from-response"
          },
          "url": `${location.href}`,
          "type": "coop"
        }
      },
    ]
  ],
  // Open a cross-origin popup with a same-origin COOP report-only value, which
  // would cause a browsing context group swap, hence a report is sent to both
  // endpoints.
  [
    CROSS_ORIGIN,
    "",
    "",
    `same-origin; report-to="${popupReportOnlyEndpoint.name}"`,
    "",
    [
      {
        "endpoint": reportOnlyEndpoint,
        "report": {
          "body": {
            "disposition": "reporting",
            "effectivePolicy": "same-origin",
            "nextResponseURL": /uuid=EXECUTOR_UUID$/, // next document URL
            "type": "navigation-from-response"
          },
          "url": `${location.href}`,
          "type": "coop"
        }
      },
      {
        "endpoint": popupReportOnlyEndpoint,
        "report": {
          "body": {
            "disposition": "reporting",
            "effectivePolicy": "same-origin",
            "previousResponseURL": "",
            "referrer": `${location.origin}/`, // referrer
            "type": "navigation-to-response"
          },
          "url": /uuid=EXECUTOR_UUID$/,
          "type": "coop"
        }
      }
    ]
  ],
  // Open a same-origin popup with a same-origin COOP report-only value, the two
  // COOP-report-only values match, hence no virtual browsing context group swap
  // happens and no report is sent.
  [
    SAME_ORIGIN,
    "",
    "",
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
        "byte_end": 32,
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
  "source_name": "html/cross-origin-opener-policy/reporting/navigation-reporting/report-only-same-origin-report-to.https.html"
}
```
