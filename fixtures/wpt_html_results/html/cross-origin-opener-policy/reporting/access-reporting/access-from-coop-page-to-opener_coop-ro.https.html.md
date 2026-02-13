# html/cross-origin-opener-policy/reporting/access-reporting/access-from-coop-page-to-opener_coop-ro.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/access-reporting/access-from-coop-page-to-opener_coop-ro.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>
  COOP reports are sent when the openee used COOP-RO+COEP and then tries to
  access its same-origin opener.
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
const redirect_path = directory + "/resources/redirect.py?";
const same_origin = get_host_info().HTTPS_ORIGIN;

let runTest = (openee_redirect, name) => promise_test(async t => {
  const report_token = token();
  const openee_token = token();

  const opener_url = location.href;

  const reportTo = reportToHeaders(report_token);
  const openee_url = same_origin + executor_path +
    reportTo.header + reportTo.coopReportOnlySameOriginHeader + coep_header +
    `&uuid=${openee_token}`;
  const openee_redirect_url = same_origin + redirect_path + openee_url
  const openee_requested_url = openee_redirect ? openee_redirect_url
                                               : openee_url;

  const openee = window.open(openee_requested_url);
  t.add_cleanup(() => send(openee_token, "window.close()"))

  // 1. Try to access the opener. A report is sent, because of COOP-RO+COEP.

  send(openee_token, addScriptAndTriggerOnload(
    directory + "/reporting/resources/try-access.js",
    "tryAccess(opener);")
  );

  // 2. Check a report is sent to the openee.
  let report =
    await receiveReport(report_token, "access-from-coop-page-to-opener")
  assert_equals(report.type, "coop");
  assert_equals(report.url, openee_url.replace(/"/g, '%22'));
  assert_equals(report.body.disposition, "reporting");
  assert_equals(report.body.effectivePolicy, "same-origin-plus-coep");
  assert_equals(report.body.property, "blur");
  assert_source_location_found(report);
  assert_equals(report.body.openerURL, opener_url);
  assert_equals(report.body.openeeURL, undefined);
  assert_equals(report.body.otherDocumentURL, undefined);
  assert_equals(report.body.referrer, opener_url);
  assert_equals(report.body.initialPopupURL, undefined);
}, name);

runTest(false, "access-from-coop-page-to-opener, same-origin");
runTest(true , "access-from-coop-page-to-opener, same-origin + redirect");

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
  "source_name": "html/cross-origin-opener-policy/reporting/access-reporting/access-from-coop-page-to-opener_coop-ro.https.html"
}
```
