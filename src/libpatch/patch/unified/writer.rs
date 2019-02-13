pub trait UnifiedPatchHunkHeaderWriter {
}

pub trait UnifiedPatchHunkWriter: UnifiedPatchHunkHeaderWriter {
impl<'a, Line> UnifiedPatchHunkHeaderWriter for Hunk<'a, Line> {
}
impl<'a> UnifiedPatchHunkWriter for Hunk<'a, LineId> {
    match () {
        #[cfg(unix)]
        () => {
            use std::os::unix::fs::PermissionsExt;

            if let Some(permissions) = filepatch.old_permissions() {
                if filepatch.kind() == FilePatchKind::Delete {
                    writeln!(writer, "delete file mode {:o}", permissions.mode())?;
                } else {
                    writeln!(writer, "old mode {:o}", permissions.mode())?;
                }
            if let Some(permissions) = filepatch.new_permissions() {
                if filepatch.kind() == FilePatchKind::Delete {
                    writeln!(writer, "new file mode {:o}", permissions.mode())?;
                } else {
                    writeln!(writer, "new mode {:o}", permissions.mode())?;
                }
        #[cfg(not(unix))]
        () => {
            // We ignore permissions.
        }