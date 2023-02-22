# Protocol handler lister

Retrieves a list of protocol handlers from the registry.
Some are fun like:
- ms-cxh-full://0
- ms-appinstaller://


## Getting Started

FizzBuzz, Fizz your Buzz, Buzz your Fizz or don't.

### Executing program

* Run
```
PS C:\Users\p4\CLionProjects\regloop> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\regloop.exe`
[i] Hunting for protocol handlers
Available
Devices
Digital
Discord
File
LDAP
MK
[...SNIP...]
xbox-store
xbox-tcui
xboxgames
xboxliveapp-1297287741
xboxmusic
zune
```

### Support
Currently only supports 
`Computer\HKEY_CLASSES_ROOT`

## References 
- https://docs.rs/winreg/latest/winreg/
- https://stackoverflow.com/questions/59645451/how-to-react-on-regvaule-types-from-winreg-crate-correctly
- https://parsiya.net/blog/2021-03-17-attack-surface-analysis-part-2-custom-protocol-handlers/

### TODO
- Fix if URL in (Default) key then take container name instead of weird slicing method 
- Add support for `Computer\HKEY_CLASSES_ROOT\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\PackageRepository\Extensions\`
- Implement checker to enum registered program / target 