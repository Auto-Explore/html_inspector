# html/cross-origin-opener-policy/reporting/navigation-reporting/reporting-popup-same-origin-allow-popups-report-to.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/navigation-reporting/reporting-popup-same-origin-allow-popups-report-to.https.html",
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
src="/html/cross-origin-opener-policy/reporting/resources/reporting-common.js?pipe=sub&report_id=6a739c25-0ec5-4832-b4a3-847281006857&report_only_id=f91209ee-b3a3-474b-b337-d663533745fb"></script>

<script>

let tests = [
  // popup origin, popup COOP, popup COEP, popup COOP report only, popup COEP report only, expected reports

  // Open a same-origin popup with a same-origin COOP and no COEP. Produces two
  // reports (one from and one to). Both pages being same origin, the
  // next/pervious document urls are available.
  [
    SAME_ORIGIN,
    `same-origin; report-to="${popupReportEndpoint.name}"`,
    "",
    "",
    "",
    [
      {
        "endpoint": reportEndpoint,
        "report": {
          "body": {
            "disposition": "enforce",
            "effectivePolicy": "same-origin-allow-popups",
            "nextResponseURL": /uuid=EXECUTOR_UUID$/, // next document URL
            "type": "navigation-from-response"
          },
          "url": `${location.href}`,
          "type": "coop"
        }
      },
      {
        "endpoint": popupReportEndpoint,
        "report": {
          "body": {
            "disposition": "enforce",
            "effectivePolicy": "same-origin",
            "previousResponseURL": `${location.href}`, // previous documnent url
            "referrer": `${location.origin}/`, // referrer (origin, as dictated by the referrer policy)
            "type": "navigation-to-response"
          },
          "url": /uuid=EXECUTOR_UUID$/,
          "type": "coop"
        }
      }
    ]
  ],
  // Open a cross-origin popup with a same-origin-allow-popup COOP and noCOEP.
  // Produces two reports (one from and one to). Both pages being cross origin,
  // the next/pervious document urls are not available and the initial document
  // url/referrer are used instead.
  [
    CROSS_ORIGIN,
    `same-origin-allow-popups; report-to="${popupReportEndpoint.name}"`,
    "require-corp",
    "",
    "",
    [
      {
        "endpoint": reportEndpoint,
        "report": {
          "body": {
            "disposition": "enforce",
            "effectivePolicy": "same-origin-allow-popups",
            "nextResponseURL": /uuid=EXECUTOR_UUID$/, // next document URL
            "type": "navigation-from-response"
          },
          "url": `${location.href}`,
          "type": "coop"
        }
      },
      {
        "endpoint": popupReportEndpoint,
        "report": {
          "body": {
            "disposition": "enforce",
            "effectivePolicy": "same-origin-allow-popups",
            "previousResponseURL": ``,
            "referrer": `${location.origin}/`, // referrer (origin, as dictated by the referrer policy)
            "type": "navigation-to-response"
          },
          "url": /uuid=EXECUTOR_UUID$/,
          "type": "coop"
        }
      }
    ]
  ],
  // Open a cross-origin popup with a same-origin COOP and COEP, and no reporting.
  // Produces one navigation-from-report for this document (the opener). The
  // pages being cross origin, the next/pervious document urls are not available
  // and the initial document url/referrer are used instead.
  [
    CROSS_ORIGIN,
    `same-origin`,
    "require-corp",
    "",
    "",
    [
      {
        "endpoint": reportEndpoint,
        "report": {
          "body": {
            "disposition": "enforce",
            "effectivePolicy": "same-origin-allow-popups",
            "nextResponseURL": /uuid=EXECUTOR_UUID$/, // initial navigation URL
            "type": "navigation-from-response"
          },
          "url": `${location.href}`,
          "type": "coop"
        }
      }
    ]
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
  "source_name": "html/cross-origin-opener-policy/reporting/navigation-reporting/reporting-popup-same-origin-allow-popups-report-to.https.html"
}
```
