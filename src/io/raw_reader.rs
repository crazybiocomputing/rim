use std::fs::File;
use std::io::Read;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}


    /** Opens the image at 'filePath' using the format specified by 'fi'. */
    public static ImagePlus open(String filePath, FileInfo fi) {
        File f = new File(filePath);
        String parent = f.getParent();
        if (parent!=null)
            fi.directory = parent+ "/";
        fi.fileName = f.getName();
        return (new FileOpener(fi)).open(false);
    }   


    /** Opens all the images in the specified directory as a stack,
        using the format specified by 'fi'. */
    public static ImagePlus openAll(String directory, FileInfo fi) {
        ImagePlus imp = openAllVirtual(directory,fi);
        if (imp!=null)
            return imp.duplicate();
        else
            return null;
    }   

    /** Opens all the images in the specified directory as a virtual stack,
        using the format specified by 'fi'. */
    public static ImagePlus openAllVirtual(String directory, FileInfo fi) {
        String[] list = new File(directory).list();
        if (list==null)
            return null;
        FolderOpener fo = new FolderOpener();
        list = fo.trimFileList(list);
        list = fo.sortFileList(list);
        if (list==null)
            return null;
        directory = IJ.addSeparator(directory);
        FileInfo[] info = new FileInfo[list.length];
        for (int i=0; i<list.length; i++) {
            info[i] = (FileInfo)fi.clone();
            info[i].directory = directory;
            info[i].fileName = list[i];
        }
        VirtualStack stack = new FileInfoVirtualStack(info);
        ImagePlus imp = new ImagePlus(directory, stack);
        return imp;
    }   
    
}
