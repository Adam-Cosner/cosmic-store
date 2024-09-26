use std::os::fd::OwnedFd;

#[cfg(feature = "logind")]
pub async fn inhibit() -> zbus::Result<Vec<OwnedFd>> {
    use logind_zbus::manager::{InhibitType, ManagerProxy};

    let connection = zbus::Connection::system().await?;
    let manager = ManagerProxy::new(&connection).await?;
    let who = "COSMIC Store";
    let why = "COSMIC Store is performing packaging operations";
    let mode = "block";
    let mut fds = Vec::new();
    for what in &[InhibitType::Shutdown, InhibitType::Sleep] {
        //TODO: update logind-zbus to fix inhibit signature
        let fd: zbus::zvariant::OwnedFd = manager
            .inner()
            .call("Inhibit", &(what, who, why, mode))
            .await?;
        // Have to convert to std type to avoid leaking zbus dependency
        fds.push(fd.into());
    }
    log::info!("{:?}", fds);
    Ok(fds)
}

#[cfg(not(feature = "logind"))]
pub async fn inhibit() -> zbus::Result<Vec<OwnedFd>> {
    Vec::new()
}
