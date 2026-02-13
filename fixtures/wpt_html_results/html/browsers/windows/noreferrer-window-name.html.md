# html/browsers/windows/noreferrer-window-name.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/noreferrer-window-name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>rel=noreferrer and reuse of names</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  async_test(function(t) {
    localStorage.clear()

    function makeHyperlink(n) {
      var hyperlink = document.body.appendChild(document.createElement("a"))
      hyperlink.rel = "noreferrer"
      hyperlink.target = "sufficientlyrandomwindownameamiright"
      hyperlink.href = "resources/noreferrer-window-name.html#" + n
      return hyperlink
    }

    var hyperlink1 = makeHyperlink(1),
        hyperlink2 = makeHyperlink(2)

    t.add_cleanup(function() {
      localStorage.setItem("x", "close")
      localStorage.clear()
      document.body.removeChild(hyperlink1)
      document.body.removeChild(hyperlink2)
    })

    addEventListener("storage", function(e) {
      t.step(function() {
        if(localStorage.getItem("window1") && localStorage.getItem("window2")) {
          localStorage.setItem("x", "close")
          t.done()
        }
      })
    })

    hyperlink1.click()
    hyperlink2.click()
  }, "Following a noreferrer link with a named target should not cause creation of a window that can be targeted by another noreferrer link with the same named target");

  async_test(function(t) {
    var ifr = document.createElement("iframe");
    ifr.name = "sufficientlyrandomwindownameamiright2";
    ifr.onload = t.step_func(function() {
      var hyperlink = document.body.appendChild(document.createElement("a"));
      t.add_cleanup(function() {
        hyperlink.remove();
      });
      hyperlink.rel = "noreferrer";
      hyperlink.href = URL.createObjectURL(new Blob(["hello subframe"],
                                                    { type: "text/html"}));
      hyperlink.target = "sufficientlyrandomwindownameamiright2";
      ifr.onload = t.step_func_done(function() {
        assert_equals(ifr.contentDocument.documentElement.textContent,
                      "hello subframe");
      });
      hyperlink.click();
    });
    document.body.appendChild(ifr);
    t.add_cleanup(function() {
      ifr.remove();
    });
  }, "Targeting a rel=noreferrer link at an existing named subframe should work");

  async_test(function(t) {
    var win = window.open("", "sufficientlyrandomwindownameamiright3");
    t.add_cleanup(() => win.close());

    const channel = new BroadcastChannel('sufficientlyrandomchannelnameamiright3');
    t.add_cleanup(() => channel.close());

    const targetHtml = `
      <script>
        const channel = new BroadcastChannel('sufficientlyrandomchannelnameamiright3');
        channel.postMessage({ name: window.name, hasOpener: window.opener === null });
      </scr`+`ipt>
    `;

    var hyperlink = document.body.appendChild(document.createElement("a"));
    t.add_cleanup(() => hyperlink.remove());
    hyperlink.rel = "noreferrer";
    hyperlink.href = URL.createObjectURL(new Blob([targetHtml],
                                                  { type: "text/html"}));
    hyperlink.target = "sufficientlyrandomwindownameamiright3";

    // win already loaded about:blank, the next load won't reuse the window. So we cannot
    // add a load listener and rather need to use a channel.
    channel.onmessage = t.step_func_done(function({ data }) {
      assert_equals(data.name, 'sufficientlyrandomwindownameamiright3');
      assert_equals(data.hasOpener, false);
      assert_equals(win.location.href, hyperlink.href);
    });

    hyperlink.click();
  }, "Targeting a rel=noreferrer link at an existing named window should work");
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
  "source_name": "html/browsers/windows/noreferrer-window-name.html"
}
```
