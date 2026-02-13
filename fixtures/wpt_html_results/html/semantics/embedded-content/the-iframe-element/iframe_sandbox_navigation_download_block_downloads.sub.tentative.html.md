# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigation_download_block_downloads.sub.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigation_download_block_downloads.sub.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Navigation resulted download in sandbox is blocked.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-iframe-sandbox">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-iframe-element">
<script src="/resources/testharness.js"></script>
<script src='/resources/testharnessreport.js'></script>
<script src="support/iframe_sandbox_download_helper.js"></script>
<body>
<script>
"use strict";

async_test(t => {
    const token = "{{$id:uuid()}}";
    var iframe = document.createElement("iframe");
    iframe.srcdoc = "<a>Download</a>";
    iframe.sandbox = "allow-same-origin";
    iframe.onload = t.step_func(function () {
        iframe.contentWindow.addEventListener(
            "unload", t.unreached_func("Unexpected navigation."));
        var anchor = iframe.contentDocument.getElementsByTagName('a')[0];
        // Set |finish-delay| to let the server stream a response over a  period
        // of time, so it's able to catch potential download cancellation by
        // detecting a socket close.
        anchor.href = "support/download_stash.py?token=" + token + "&finish-delay=" + StreamDownloadFinishDelay();
        anchor.click();
        AssertDownloadFailure(t, token, StreamDownloadFinishDelay() + DownloadVerifyDelay());
    });

    document.body.appendChild(iframe);
}, "Navigation resulted download in sandbox is blocked.");


async_test(t => {
    const token = "{{$id:uuid()}}";
    var iframe = document.createElement("iframe");

    const folder = location.origin+"/html/semantics/embedded-content/the-iframe-element/";
    const href = `${folder}support/download_stash.py?token=${token}&finish-delay=${StreamDownloadFinishDelay() }`;
    const objectDoc =`<a href="${href}">download</a>
    <${"script"}> document.querySelector("a").click(); </${"script"}>`;

    iframe.srcdoc = `<object data='data:text/html,${objectDoc}'></object>`;
    iframe.sandbox = "allow-same-origin  allow-scripts";
    iframe.addEventListener("load",()=>{
        AssertDownloadFailure(t, token, StreamDownloadFinishDelay() + DownloadVerifyDelay());
    })
    document.body.appendChild(iframe);
}, "Navigation resulted download in sandbox from <object> is blocked.");

</script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigation_download_block_downloads.sub.tentative.html"
}
```
