pipeline {
  agent {
    docker { image 'rust:latest' }
  }
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
        archiveArtifacts 'target/debug/typos-json-to-github-annotations'
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

    stage('Clippy') {
      steps {
        sh '''
          rustup component add clippy
          cargo clippy --all
        '''
      }
    }

    stage('Rustfmt') {
      steps {
        sh '''
          rustup component add rustfmt
          cargo fmt --all -- check
        '''
      }
      post {
        always {
          recordIssues tools: [cargo()]
        }
      }
    }

    stage('Doc') {
      steps {
        // Not sure what to do with this yet
        sh 'cargo doc'
      }
    }
  }
}
