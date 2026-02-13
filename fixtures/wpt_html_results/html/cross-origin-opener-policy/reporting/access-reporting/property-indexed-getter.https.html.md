# html/cross-origin-opener-policy/reporting/access-reporting/property-indexed-getter.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/reporting/access-reporting/property-indexed-getter.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title> Check reports are sent for the indexed getter</title>
<meta name=timeout content=long>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/get-host-info.sub.js></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script>

const directory = "/html/cross-origin-opener-policy";
const executor_path = "/common/dispatcher/executor.html?pipe=";
const coep_header = '|header(Cross-Origin-Embedder-Policy,require-corp)';

let origin = [
  ["cross-origin" , get_host_info().HTTPS_REMOTE_ORIGIN ] ,
  ["same-site"    , get_host_info().HTTPS_ORIGIN        ] ,
];

let testCase = [
//[operation  , expectReport ] ,
  [w => w[0]  , true         ], // Existing iframe.
  [w => w[1]  , false        ], // Out of bounds (positive).
  [w => w[-1] , false        ], // Out of bounds (negative).
];

origin.forEach(([origin_name, origin]) => {
  testCase.forEach(([op, expectReport]) =>  {
    promise_test(async t => {
      const opener_token = token();
      const openee_token = token();

      const openee_url = origin+ executor_path + `&uuid=${openee_token}`;
      const openee = window.open(openee_url);
      t.add_cleanup(() => send(openee_token, "window.close()"))

      // 1. Create an iframe in the openee.
      send(openee_token, `
        let iframe = document.createElement("iframe");
        document.body.appendChild(iframe);

        send("${opener_token}", "openee loaded");
      `);
      let reply = await receive(opener_token);
      assert_equals(reply, "openee loaded");

      // 2. Try to access the openee.
      let observer = new ReportingObserver(()=>{});
      observer.observe();
      try {op(openee)} catch(e) {}
      let reports = observer.takeRecords();
      observer.disconnect();

      // 3. Check the received reports.
      if (expectReport) {
        assert_equals(reports.length, 1);
        assert_equals(reports[0].type, "coop-access-violation");
        assert_equals(reports[0].body.property, "indexed");
      } else {
        assert_equals(reports.length, 0);
      }

    }, `${origin_name} > ${op}`);
});
});

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
  "source_name": "html/cross-origin-opener-policy/reporting/access-reporting/property-indexed-getter.https.html"
}
```
