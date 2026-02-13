# html/cross-origin-opener-policy/reporting/access-reporting/access-from-coop-page-to-other_coop-ro_cross-origin.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/access-reporting/access-from-coop-page-to-other_coop-ro_cross-origin.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>
  One window accesses a second one. They are aren't related by an opener/openee
  relationship. The first window has set
  Cross-Origin-Opener-Policy-Report-Only:same-origin, so it receives a
  "access-from-coop-page-to-other" report.
</title>
<meta name=timeout content=long>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/get-host-info.sub.js></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/cross-origin-opener-policy/resources/common.js"></script>
<script src="/html/cross-origin-opener-policy/reporting/resources/reporting-common.js"></script>
<script src="/html/cross-origin-opener-policy/reporting/resources/try-access.js"></script>
<script>

const directory = "/html/cross-origin-opener-policy";
const same_origin= get_host_info().HTTPS_ORIGIN;
const cross_origin= get_host_info().HTTPS_REMOTE_ORIGIN;

let escapeComma = url => url.replace(/,/g, '\\,');

promise_test(async t => {
  const report_token= token();
  const report_to = reportToHeaders(report_token);

  // The test window.
  const this_window_token = token();

  // The "opener" window. With COOP:same-origin + reporter.
  const opener_token = token();
  const opener_url = same_origin + executor_path + report_to.header +
    report_to.coopReportOnlySameOriginHeader + coep_header +
    `&uuid=${opener_token}`;

  // The "openee" window. With COOP:same-origin + reporter.
  const openee_token = token();
  const openee_url = same_origin + executor_path + report_to.header +
    report_to.coopReportOnlySameOriginHeader + coep_header +
    `&uuid=${openee_token}`;

  // The "other" window.
  const other_token = token();
  const other_url = cross_origin + executor_path + report_to.header +
    `&uuid=${other_token}`;

  t.add_cleanup(() => {
    send(opener_token, "window.close()")
    send(openee_token, "window.close()")
    send(other_token, "window.close()")
  })

  // 1. Create the "opener" window.
  let opener_window_proxy = window.open(opener_url);

  // 2. Create the "openee" window.
  send(opener_token, `
    window.openee = window.open('${escapeComma(openee_url)}');
  `);

  // 3. Create the "other" window.
  send(openee_token, `
    window.other = window.open('${escapeComma(other_url)}');
  `);

  // 4. Wait for "other" to load its document.
  send(other_token, `send('${this_window_token}', "Loaded");`);
  assert_equals(await receive(this_window_token), "Loaded");

  // 5. "opener" accesses "other" window, through "openee".

  send(opener_token, addScriptAndTriggerOnload(
    directory + "/reporting/resources/try-access.js",
    "tryAccess(openee.other);")
  );

  // 6. Check a report is sent to the openee.
  let report =
    await receiveReport(report_token, "access-from-coop-page-to-other")
  assert_equals(report.type, "coop");
  assert_equals(report.url, opener_url.replace(/"/g, '%22'));
  assert_equals(report.body.disposition, "reporting");
  assert_equals(report.body.effectivePolicy, "same-origin-plus-coep");
  assert_equals(report.body.property, "blur");
  assert_source_location_found(report);
  assert_equals(report.body.openerURL, undefined);
  assert_equals(report.body.openeeURL, undefined);
  assert_equals(report.body.otherDocumentURL, "");
  assert_equals(report.body.referrer, undefined);
}, "access-from-coop-page-to-other (COOP-RO)");

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
  "source_name": "html/cross-origin-opener-policy/reporting/access-reporting/access-from-coop-page-to-other_coop-ro_cross-origin.https.html"
}
```
