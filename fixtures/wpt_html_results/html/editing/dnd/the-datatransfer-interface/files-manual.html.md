# html/editing/dnd/the-datatransfer-interface/files-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-datatransfer-interface/files-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
    <head>
        <title>HTML5 Drag and Drop: files attribute returns a FileList</title>
        <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/"/>
        <link rel="help" href="http://dev.w3.org/html5/spec/dnd.html#datatransfer"/>
        <meta name="assert" content="files attribute returns a FileList"/>
        <script type="text/javascript">
            var EVENT, TARGET;

            function DropEvent(evt)
            {
                if ((TARGET == evt.target) && (EVENT == evt.type))
                {
                    var files = evt.dataTransfer.files;
                    if(('[object FileList]' == files))
                    {
                      document.getElementById("test_result").firstChild.data = "PASS";
                    }
                    else
                    {
                      document.getElementById("test_result").firstChild.data = "FAIL";
                    }
                }
                else
                {
                  document.getElementById("test_result").firstChild.data = "FAIL";
                }
            }

            function DragenterEvent(evt)
            {
                evt.preventDefault();
            }

            function DragoverEvent(evt)
            {
                evt.preventDefault();
            }

            EVENT = "drop";

            window.onload = function()
            {
                TARGET = document.getElementById("target");
                TARGET.addEventListener(EVENT, DropEvent, false);
                TARGET.addEventListener("dragenter", DragenterEvent, false);
                TARGET.addEventListener("dragover", DragoverEvent, false);
            }
        </script>
    </head>
    <body>
        <pre>Description: files attribute returns a FileList</pre>
        <table id='testtable' border='1'>
            <tr>
                <td>Test Result</td>
                <td>Test Assertion</td>
            </tr>
            <tr>
                <td id='test_result'>Manual</td>
                <td id='test_assertion'>Test passes if if the word "PASS" appears to the left after following the steps below.
                <div id="manualsteps">
                    Steps:
                    <ol>
                        <li> Drag a file and drop it in the green box
                    </ol>
                </div>
            </td>
            </tr>
        </table>
        <p>
        http://dev.w3.org/html5/spec/dnd.html#datatransfer
        </p>
        <p>
        The files attribute must return a live FileList sequence consisting of File objects representing the files.
        </p>
        <textarea type="text" id="target" style="border:2px green solid; width:200px; height:50px"></textarea>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 469,
        "byte_start": 438,
        "col": 9,
        "line": 9
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
  "source_name": "html/editing/dnd/the-datatransfer-interface/files-manual.html"
}
```
