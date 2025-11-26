# iOS Security Guard (Rust Edition)

🛡️ **iOS Security Guard** 是一个基于 Rust 编写的高性能 iOS 防破解、防调试、防注入静态库。它利用 Rust 的内存安全特性和底层系统调用能力，为您的 iOS 应用提供坚实的运行时保护。

## ✨ 核心功能 (Features)

1.  **🚫 反调试 (Anti-Debug)**
    *   **主动防御**: 使用 `ptrace(PT_DENY_ATTACH)` 主动拒绝调试器挂载。
    *   **时间检测**: 通过 `mach_absolute_time` 检测代码执行耗时，识别单步调试 (Single-Stepping) 行为。

2.  **💉 反注入 (Anti-Injection)**
    *   **动态库检测**: 实时扫描已加载的动态库，识别 Frida, CydiaSubstrate, SSLKillSwitch 等常见越狱/Hook 框架。
    *   **字符串混淆**: 关键检测字符串（如 "Frida"）在编译时自动加密 (Obfuscated)，防止通过 `strings` 命令或 IDA 静态搜索定位检测逻辑。

3.  **🔒 完整性校验 (Integrity)**
    *   **反 Patch**: 在运行时检查关键函数的机器码头部，防止攻击者通过修改汇编指令（如 `RET` 或 `B` 跳转）来绕过防护。

4.  **💥 汇编级崩溃 (Assembly Trap)**
    *   **不可恢复**: 一旦检测到威胁，直接执行 `brk #1` 内联汇编指令触发硬件中断。
    *   **绕过 Hook**: 这种崩溃方式不经过 `exit()` 或 `abort()`，因此无法被常规的 Hook 手段拦截。

## 🚀 优势 (Pros)

*   **高性能**: 基于 Rust 编写，无 GC 开销，对 App 启动速度和运行性能影响微乎其微。
*   **内存安全**: Rust 的所有权机制杜绝了空指针、缓冲区溢出等常见安全漏洞。
*   **难以逆向**: 相比 Objective-C 的动态派发，Rust 编译后的二进制文件去除了大量符号信息，且逻辑更加紧凑，增加了逆向分析的难度。
*   **易于集成**: 提供标准的 `.xcframework`，支持 iOS 真机 (arm64) 和模拟器 (arm64)。

## ⚠️ 局限性 (Cons)

*   **非银弹**: 客户端的安全防护永远是“猫鼠游戏”。经验丰富的逆向工程师总有办法绕过（例如修改内核、使用硬件调试器等）。本库旨在大幅增加破解成本，而非绝对防御。
*   **App Store 风险**: 使用 `ptrace` 等私有 API 可能会在 App Store 审核中遇到阻碍（尽管许多商业安全 SDK 也在使用类似技术，建议混淆调用或在提交审核版中通过宏关闭）。
*   **控制流混淆**: 标准 Rust 编译器不支持控制流平坦化 (Control Flow Flattening)。如果需要极致的逻辑混淆，建议配合 Obfuscator-LLVM 使用。

## 🛠 使用教程 (Usage)

### 1. 编译 (Build)

确保已安装 Rust 工具链 (`rustup`, `cargo`)。

```bash
# 克隆项目
git clone https://github.com/your-repo/ios-security-guard.git
cd ios-security-guard

# 运行构建脚本
./build_xcframework.sh
```

构建成功后，将在当前目录生成 `ios_security_guard.xcframework` 和 `IOSSecurityGuard.h`。

### 2. 集成 (Integration)

1.  将 `ios_security_guard.xcframework` 拖入您的 Xcode 项目。
2.  在 Target 设置 -> **General** -> **Frameworks, Libraries, and Embedded Content** 中，将该库设置为 **"Embed & Sign"**。
3.  将 `IOSSecurityGuard.h` 添加到项目中。

### 3. 调用 (Call)

**Objective-C:**

```objective-c
#import "IOSSecurityGuard.h"

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    start_protection();
    return YES;
}
```

**Swift:**

在 `Bridging-Header.h` 中导入：
```objective-c
#import "IOSSecurityGuard.h"
```

在 `AppDelegate.swift` 中调用：
```swift
func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
    // 启动防护，若检测到威胁将直接崩溃
    start_protection()
    return true
}
```

## ⚖️ 免责声明 (Disclaimer)

本项目仅供安全研究和教育目的使用。开发者不对使用本项目造成的任何应用审核拒绝、数据丢失或其他后果负责。请在遵守当地法律法规的前提下使用。
