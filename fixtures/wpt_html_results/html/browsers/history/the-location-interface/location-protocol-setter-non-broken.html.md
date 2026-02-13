# html/browsers/history/the-location-interface/location-protocol-setter-non-broken.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-protocol-setter-non-broken.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Set location.protocol to a non-broken-non-functioning scheme</title>
<!-- In particular, valid non-broken schemes that are nevertheless not going to work -->
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
self.onload = () => {
  [
    'x',
    'data',
    // 'mailto' opens an email client in Chrome and Firefox and then ends up passing anyway...
    'file',
    'ftp',
    'http+x'
  ].forEach((val) => {
    async_test((t) => {
      // HTTP URL <iframe>
      const frame = document.createElement("iframe")
      t.add_cleanup(() => frame.remove())
      frame.src = "/common/blank.html"
      frame.onload = t.step_func(() => {
        frame.contentWindow.location.protocol = val
        t.step_timeout(() => {
          assert_equals(frame.contentWindow.location.protocol, location.protocol)
          assert_equals(frame.contentWindow.location.host, location.host)
          assert_equals(frame.contentWindow.location.port, location.port)
          t.done()
          // Matches the timeout from location-protocol-setter-non-broken-weird.html which suggests
          // that 4 seconds is enough for a navigation to complete.
        }, 4000)
      })
      document.body.appendChild(frame)
    }, "Set HTTP URL frame location.protocol to " + val)

    async_test((t) => {
      // data URL <iframe>
      const dataFrame = document.createElement("iframe")
      t.add_cleanup(() => dataFrame.remove())
      const channel = new MessageChannel()
      dataFrame.src = `data:text/html,<script>
onmessage = (e) => {
  let result = false;
  try {
    location.protocol = e.data
  } catch(e) {
    result = true
  }
  setTimeout(() => e.ports[0].postMessage([result, location.protocol]), 4000)
}
<\/script>`
      dataFrame.onload = t.step_func(() => {
        dataFrame.contentWindow.postMessage(val, "*", [channel.port2])
      })
      channel.port1.onmessage = t.step_func_done((e) => {
        assert_false(e.data[0])
        assert_equals(e.data[1], "data:")
      })
      document.body.appendChild(dataFrame)
    }, "Set data URL frame location.protocol to " + val)
  })
}
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
  "source_name": "html/browsers/history/the-location-interface/location-protocol-setter-non-broken.html"
}
```
